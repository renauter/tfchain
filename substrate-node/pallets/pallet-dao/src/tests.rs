use super::Event as DaoEvent;
use crate::{mock::Event as MockEvent, mock::*, Error};
use frame_support::{assert_noop, assert_ok, weights::GetDispatchInfo};
use frame_system::{EventRecord, Phase, RawOrigin};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};
use tfchain_support::types::{FarmCertification, Location, NodeCertification, PublicIP, Resources};

#[test]
fn farmers_vote_no_farm_fails() {
    new_test_ext().execute_with(|| {
        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        let _proposal_weight = proposal.get_dispatch_info().weight;
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            3,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        assert_noop!(
            DaoModule::vote(Origin::signed(10), 1, hash.clone(), true),
            Error::<Test>::NotAuthorizedToVote
        );
    });
}

#[test]
fn farmers_vote_proposal_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        let _proposal_weight = proposal.get_dispatch_info().weight;
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            3,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        prepare_twin_farm_and_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), true));
    });
}

#[test]
fn farmers_vote_proposal_if_no_nodes_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        let _proposal_weight = proposal.get_dispatch_info().weight;
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            1,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        prepare_twin(10);
        prepare_farm(10, "f1".as_bytes().to_vec());
        assert_noop!(
            DaoModule::vote(Origin::signed(10), 1, hash.clone(), true),
            Error::<Test>::FarmHasNoNodes
        );

        let votes = DaoModule::voting(hash).unwrap();
        assert_eq!(votes.ayes.len(), 0);
    });
}

#[test]
fn farmer_weight_is_set_properly_when_node_is_added_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);

        let weight = DaoModule::farm_weight(1);
        assert_ne!(weight, 0);
    });
}

#[test]
fn farmer_weight_is_set_properly_when_node_is_added_and_removed_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);

        let weight = DaoModule::farm_weight(1);
        assert_ne!(weight, 0);

        TfgridModule::delete_node_farm(Origin::signed(10), 1).unwrap();

        let weight = DaoModule::farm_weight(1);
        assert_eq!(weight, 0);
    });
}

#[test]
fn close_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        // Farmer 1 votes yes
        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        // Farmer 2 votes yes
        prepare_twin_farm_and_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), true));

        System::set_block_number(4);
        assert_noop!(
            DaoModule::close(Origin::signed(4), hash.clone(), 0,),
            Error::<Test>::TooEarly
        );

        System::set_block_number(5);
        assert_ok!(DaoModule::close(Origin::signed(4), hash.clone(), 0,));

        let e = System::events();
        assert_eq!(
            e[4],
            record(MockEvent::DaoModule(DaoEvent::Proposed {
                account: 1,
                proposal_index: 0,
                proposal_hash: hash,
                threshold: 2
            }))
        );

        assert_eq!(
            e[8],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 10,
                proposal_hash: hash,
                voted: true,
                yes: 1,
                no: 0
            }))
        );

        assert_eq!(
            e[12],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 11,
                proposal_hash: hash,
                voted: true,
                yes: 2,
                no: 0
            }))
        );

        let farm_1_weight = DaoModule::get_vote_weight(1).unwrap();
        let farm_2_weight = DaoModule::get_vote_weight(2).unwrap();
        let total_weight = farm_1_weight + farm_2_weight;

        assert_eq!(
            e[13],
            record(MockEvent::DaoModule(DaoEvent::Closed {
                proposal_hash: hash,
                yes: 2,
                yes_weight: total_weight,
                no: 0,
                no_weight: 0,
            }))
        );

        assert_eq!(
            e[14],
            record(MockEvent::DaoModule(DaoEvent::Approved {
                proposal_hash: hash,
            }))
        );

        let proposals = DaoModule::proposals_list_hashes();
        assert_eq!(proposals.len(), 0);
    });
}

#[test]
fn motion_approval_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = Call::TfgridModule(pallet_tfgrid::Call::set_farm_certification {
            farm_id: 1,
            certification: FarmCertification::Gold,
        });
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        // Farmer 1 votes yes
        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        // Farmer 2 votes yes
        prepare_twin_farm_and_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), true));

        // // Check farm certification type before we close
        let f1 = TfgridModule::farms(1);
        assert_eq!(f1.certification, FarmCertification::NotCertified);

        System::set_block_number(5);
        assert_ok!(DaoModule::close(Origin::signed(4), hash.clone(), 0,));

        let e = System::events();
        assert_eq!(
            e[4],
            record(MockEvent::DaoModule(DaoEvent::Proposed {
                account: 1,
                proposal_index: 0,
                proposal_hash: hash,
                threshold: 2
            }))
        );

        assert_eq!(
            e[8],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 10,
                proposal_hash: hash,
                voted: true,
                yes: 1,
                no: 0
            }))
        );

        assert_eq!(
            e[12],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 11,
                proposal_hash: hash,
                voted: true,
                yes: 2,
                no: 0
            }))
        );

        let farm_1_weight = DaoModule::get_vote_weight(1).unwrap();
        let farm_2_weight = DaoModule::get_vote_weight(2).unwrap();
        let total_weight = farm_1_weight + farm_2_weight;

        assert_eq!(
            e[13],
            record(MockEvent::DaoModule(DaoEvent::Closed {
                proposal_hash: hash,
                yes: 2,
                yes_weight: total_weight,
                no: 0,
                no_weight: 0,
            }))
        );

        assert_eq!(
            e[14],
            record(MockEvent::DaoModule(DaoEvent::Approved {
                proposal_hash: hash,
            }))
        );

        assert_eq!(
            e[16],
            record(MockEvent::DaoModule(DaoEvent::Executed {
                proposal_hash: hash,
                result: Ok(())
            }))
        );

        // // FarmCertification type of farm should be set to certified.
        let f1 = TfgridModule::farms(1);
        assert_eq!(f1.certification, FarmCertification::Gold);
    });
}

#[test]
fn motion_veto_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = Call::TfgridModule(pallet_tfgrid::Call::set_farm_certification {
            farm_id: 1,
            certification: FarmCertification::Gold,
        });
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        assert_ok!(DaoModule::veto(Origin::signed(2), hash.clone()));
        assert_ok!(DaoModule::veto(Origin::signed(3), hash.clone()));

        System::set_block_number(5);

        let e = System::events();
        assert_eq!(
            e[4],
            record(MockEvent::DaoModule(DaoEvent::Proposed {
                account: 1,
                proposal_index: 0,
                proposal_hash: hash,
                threshold: 2
            }))
        );

        for event in e.clone() {
            println!("event: {:?}", event);
        }

        assert_eq!(
            e[5],
            record(MockEvent::DaoModule(DaoEvent::CouncilMemberVeto {
                proposal_hash: hash,
                who: 2,
            }))
        );
        assert_eq!(
            e[6],
            record(MockEvent::DaoModule(DaoEvent::CouncilMemberVeto {
                proposal_hash: hash,
                who: 3,
            }))
        );

        assert_eq!(
            e[7],
            record(MockEvent::DaoModule(DaoEvent::ClosedByCouncil {
                proposal_hash: hash,
                vetos: [2, 3].to_vec(),
            }))
        );

        assert_eq!(
            e[8],
            record(MockEvent::DaoModule(DaoEvent::Disapproved {
                proposal_hash: hash,
            }))
        );

        // assert_eq!(
        //     e[8],
        //     record(MockEvent::DaoModule(DaoEvent::Voted {
        //         account: 11,
        //         proposal_hash: hash,
        //         voted: true,
        //         yes: 2,
        //         no: 0
        //     }))
        // );

        // let farm_1_weight = DaoModule::get_vote_weight(1).unwrap();
        // let farm_2_weight = DaoModule::get_vote_weight(2).unwrap();
        // let total_weight = farm_1_weight + farm_2_weight;

        // assert_eq!(
        //     e[9],
        //     record(MockEvent::DaoModule(DaoEvent::Closed {
        //         proposal_hash: hash,
        //         yes: 2,
        //         yes_weight: total_weight,
        //         no: 0,
        //         no_weight: 0,
        //     }))
        // );

        // assert_eq!(
        //     e[10],
        //     record(MockEvent::DaoModule(DaoEvent::Approved {
        //         proposal_hash: hash,
        //     }))
        // );
    });
}

#[test]
fn weighted_voting_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = Call::TfgridModule(pallet_tfgrid::Call::set_farm_certification {
            farm_id: 1,
            certification: FarmCertification::Gold,
        });
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        // Farmer 1 votes yes
        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        // Farmer 2 votes no
        prepare_twin_farm_and_big_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), false));

        // // Check farm certification type before we close
        let f1 = TfgridModule::farms(1);
        assert_eq!(f1.certification, FarmCertification::NotCertified);

        System::set_block_number(5);
        assert_ok!(DaoModule::close(Origin::signed(4), hash.clone(), 0,));

        let e = System::events();
        assert_eq!(
            e[4],
            record(MockEvent::DaoModule(DaoEvent::Proposed {
                account: 1,
                proposal_index: 0,
                proposal_hash: hash,
                threshold: 2
            }))
        );

        assert_eq!(
            e[8],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 10,
                proposal_hash: hash,
                voted: true,
                yes: 1,
                no: 0
            }))
        );

        assert_eq!(
            e[12],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 11,
                proposal_hash: hash,
                voted: false,
                yes: 1,
                no: 1
            }))
        );

        let farm_1_weight = DaoModule::get_vote_weight(1).unwrap();
        let farm_2_weight = DaoModule::get_vote_weight(2).unwrap();

        assert_eq!(
            e[13],
            record(MockEvent::DaoModule(DaoEvent::Closed {
                proposal_hash: hash,
                yes: 1,
                yes_weight: farm_1_weight,
                no: 1,
                no_weight: farm_2_weight,
            }))
        );

        // Outcome should be negative since the 2nd farmer which has more weight because he
        // has more stake in the network voted no
        assert_eq!(
            e[14],
            record(MockEvent::DaoModule(DaoEvent::Disapproved {
                proposal_hash: hash,
            }))
        );

        // FarmCertification type of farm should still be the same.
        let f1 = TfgridModule::farms(1);
        assert_eq!(f1.certification, FarmCertification::NotCertified);
    });
}

#[test]
fn voting_tfgridmodule_call_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = Call::TfgridModule(pallet_tfgrid::Call::set_connection_price { price: 100 });
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            None
        ));

        // Farmer 1 votes yes
        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        // Farmer 2 votes no
        prepare_twin_farm_and_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), true));

        // Check connection price of node 1
        let n1 = TfgridModule::nodes(1);
        assert_eq!(n1.connection_price, 80);

        System::set_block_number(5);
        assert_ok!(DaoModule::close(Origin::signed(4), hash.clone(), 0,));

        let e = System::events();
        for (idx, event) in e.clone().iter().enumerate() {
            println!("index: {:?}, event: {:?}", idx, event);
        }
        assert_eq!(
            e[4],
            record(MockEvent::DaoModule(DaoEvent::Proposed {
                account: 1,
                proposal_index: 0,
                proposal_hash: hash,
                threshold: 2
            }))
        );

        assert_eq!(
            e[8],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 10,
                proposal_hash: hash,
                voted: true,
                yes: 1,
                no: 0
            }))
        );

        assert_eq!(
            e[12],
            record(MockEvent::DaoModule(DaoEvent::Voted {
                account: 11,
                proposal_hash: hash,
                voted: true,
                yes: 2,
                no: 0
            }))
        );

        let farm_1_weight = DaoModule::get_vote_weight(1).unwrap();
        let farm_2_weight = DaoModule::get_vote_weight(2).unwrap();
        let total_weight = farm_1_weight + farm_2_weight;

        assert_eq!(
            e[13],
            record(MockEvent::DaoModule(DaoEvent::Closed {
                proposal_hash: hash,
                yes: 2,
                yes_weight: total_weight,
                no: 0,
                no_weight: 0,
            }))
        );

        assert_eq!(
            e[14],
            record(MockEvent::DaoModule(DaoEvent::Approved {
                proposal_hash: hash,
            }))
        );

        assert_eq!(
            e[16],
            record(MockEvent::DaoModule(DaoEvent::Executed {
                proposal_hash: hash,
                result: Ok(())
            }))
        );

        // Connection price should have been modified, any new node should have set the new price
        prepare_twin(15);
        prepare_node(15, 1);
        let n3 = TfgridModule::nodes(3);
        assert_eq!(n3.connection_price, 100);
    });
}

#[test]
fn customize_proposal_duration_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();

        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        let hash = BlakeTwo256::hash_of(&proposal);

        assert_ok!(DaoModule::propose(
            Origin::signed(1),
            2,
            Box::new(proposal.clone()),
            "some_description".as_bytes().to_vec(),
            "some_link".as_bytes().to_vec(),
            Some(10)
        ));

        // Farmer 1 votes yes
        prepare_twin_farm_and_node(10, "f1".as_bytes().to_vec(), 1);
        assert_ok!(DaoModule::vote(Origin::signed(10), 1, hash.clone(), true));

        // Farmer 2 votes yes
        prepare_twin_farm_and_node(11, "f2".as_bytes().to_vec(), 2);
        assert_ok!(DaoModule::vote(Origin::signed(11), 2, hash.clone(), true));

        System::set_block_number(9);
        assert_noop!(
            DaoModule::close(Origin::signed(4), hash.clone(), 0,),
            Error::<Test>::TooEarly
        );

        System::set_block_number(11);
        assert_ok!(DaoModule::close(Origin::signed(4), hash.clone(), 0,));
    });
}

#[test]
fn customize_proposal_duration_out_of_bounds_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        create_farming_policies();
        let proposal = make_proposal("some_remark".as_bytes().to_vec());
        // assert_noop!(
        //     DaoModule::propose(
        //         Origin::signed(1),
        //         2,
        //         Box::new(proposal.clone()),
        //         "some_description".as_bytes().to_vec(),
        //         "some_link".as_bytes().to_vec(),
        //         Some(1000000000)
        //     ),
        //     Error::<Test>::InvalidProposalDuration
        // );

        assert_noop!(
            DaoModule::propose(
                Origin::signed(1),
                2,
                Box::new(proposal.clone()),
                "some_description".as_bytes().to_vec(),
                "some_link".as_bytes().to_vec(),
                Some(1000000000)
            ),
            Error::<Test>::InvalidProposalDuration
        );
    });
}

fn record(event: Event) -> EventRecord<Event, H256> {
    EventRecord {
        phase: Phase::Initialization,
        event,
        topics: vec![],
    }
}

fn make_proposal(value: Vec<u8>) -> Call {
    Call::System(frame_system::Call::remark { remark: value })
}

pub fn prepare_twin_farm_and_node(account_id: u64, farm_name: Vec<u8>, farm_id: u32) {
    prepare_twin(account_id);
    prepare_farm(account_id, farm_name);
    prepare_node(account_id, farm_id);
}

pub fn prepare_twin_farm_and_big_node(account_id: u64, farm_name: Vec<u8>, farm_id: u32) {
    prepare_twin(account_id);
    prepare_farm(account_id, farm_name);
    prepare_big_node(account_id, farm_id);
}

pub fn prepare_twin(account_id: u64) {
    let document = "some_link".as_bytes().to_vec();
    let hash = "some_hash".as_bytes().to_vec();

    assert_ok!(TfgridModule::user_accept_tc(
        Origin::signed(account_id),
        document.clone(),
        hash.clone(),
    ));
    let ip = "10.2.3.3";
    TfgridModule::create_twin(Origin::signed(account_id), ip.as_bytes().to_vec()).unwrap();
}

const GIGABYTE: u64 = 1024 * 1024 * 1024;
fn prepare_node(account_id: u64, farm_id: u32) {
    // random location
    let location = Location {
        longitude: "12.233213231".as_bytes().to_vec(),
        latitude: "32.323112123".as_bytes().to_vec(),
    };

    let resources = Resources {
        hru: 1024 * GIGABYTE,
        sru: 512 * GIGABYTE,
        cru: 8,
        mru: 16 * GIGABYTE,
    };

    let country = "Belgium".as_bytes().to_vec();
    let city = "Ghent".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_node(
        Origin::signed(account_id),
        farm_id,
        resources,
        location,
        country,
        city,
        Vec::new(),
        false,
        false,
        "some_serial".as_bytes().to_vec(),
    ));
}

fn prepare_big_node(account_id: u64, farm_id: u32) {
    // random location
    let location = Location {
        longitude: "12.233213231".as_bytes().to_vec(),
        latitude: "32.323112123".as_bytes().to_vec(),
    };

    let resources = Resources {
        hru: 20024 * GIGABYTE,
        sru: 2024 * GIGABYTE,
        cru: 16,
        mru: 64 * GIGABYTE,
    };

    let country = "Belgium".as_bytes().to_vec();
    let city = "Ghent".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_node(
        Origin::signed(account_id),
        farm_id,
        resources,
        location,
        country,
        city,
        Vec::new(),
        false,
        false,
        "some_serial".as_bytes().to_vec(),
    ));
}

pub fn prepare_farm(account_id: u64, farm_name: Vec<u8>) {
    let mut pub_ips = Vec::new();
    pub_ips.push(PublicIP {
        ip: "1.1.1.0".as_bytes().to_vec(),
        gateway: "1.1.1.1".as_bytes().to_vec(),
        contract_id: 0,
    });

    TfgridModule::create_farm(Origin::signed(account_id), farm_name, pub_ips.clone()).unwrap();
}

fn create_farming_policies() {
    let name = "f1".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_farming_policy(
        RawOrigin::Root.into(),
        name,
        12,
        15,
        10,
        8,
        9999,
        System::block_number() + 100,
        true,
        true,
        NodeCertification::Diy,
        FarmCertification::Gold,
    ));

    let name = "f2".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_farming_policy(
        RawOrigin::Root.into(),
        name,
        12,
        15,
        10,
        8,
        9999,
        System::block_number() + 100,
        true,
        true,
        NodeCertification::Diy,
        FarmCertification::NotCertified,
    ));

    let name = "f3".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_farming_policy(
        RawOrigin::Root.into(),
        name,
        12,
        15,
        10,
        8,
        9999,
        System::block_number() + 100,
        true,
        true,
        NodeCertification::Certified,
        FarmCertification::Gold,
    ));

    let name = "f1".as_bytes().to_vec();
    assert_ok!(TfgridModule::create_farming_policy(
        RawOrigin::Root.into(),
        name,
        12,
        15,
        10,
        8,
        9999,
        System::block_number() + 100,
        true,
        true,
        NodeCertification::Certified,
        FarmCertification::NotCertified,
    ));
}