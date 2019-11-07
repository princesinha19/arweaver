use arweave::*;
mod settings;

#[test]
fn info() {
    let c = Client::new().unwrap();
    let i = c.info().unwrap();
    assert!(i.height >= settings::recent_block_height());
}

#[test]
fn block() {
    let c = Client::new().unwrap();
    let i = c.info().unwrap();
    let b0 = c.block(&i.current).unwrap();
    assert_eq!(b0.indep, i.current);
    assert_eq!(i.height, b0.height);

    let b1 = c.block(b0.previous_block().unwrap()).unwrap();
    assert_eq!(Some(&b1.indep), b0.previous_block());
    assert_eq!(b1.height + Height::from(1), b0.height);
    assert!(b1.timestamp < b0.timestamp);
}

#[test]
fn current_block() {
    let c = Client::new().unwrap();
    let b0 = c.current_block().unwrap();
    assert!(b0.timestamp >= settings::recent_block_timestamp());
    let b1 = c.block(&b0.indep).unwrap();
    assert_eq!(b0.indep, b1.indep);
    assert_eq!(b0.height, b1.height);
    assert_eq!(b1.timestamp, b0.timestamp);
}

#[test]
fn height() {
    let c = Client::new().unwrap();
    let i = c.info().unwrap();

    let b0 = c.height(i.height).unwrap();
    assert_eq!(b0.indep, i.current);
    assert_eq!(b0.height, i.height);

    let b1 = c.height(i.height - Height::from(1)).unwrap();
    assert_eq!(Some(&b1.indep), b0.previous_block());
    assert_eq!(b1.height + Height::from(1), b0.height);
    assert!(b1.timestamp < b0.timestamp);
}

#[test]
fn genesis_block() {
    let c = Client::new().unwrap();
    let b = c.height(Height::from(0)).unwrap();
    assert!(b.previous_block().is_none());
}

#[test]
fn txs() {
    let c = Client::new().unwrap();
    let (bh, ts) = settings::block_with_transactions();

    let b = c.block(bh).unwrap();
    assert_eq!(b.timestamp, ts);

    assert_ne!(b.txs.len(), 0);
    for txh in b.txs {
        let tx = c.tx(&txh).unwrap();
        assert_eq!(txh, tx.id);
    }
}

#[test]
fn tx_data_style() {
    let c = Client::new().unwrap();
    let (txh, r) = settings::data_transaction();
    let t = c.tx(&txh).unwrap();
    assert_eq!(t.id, txh);
    assert_ne!(t.data.len(), 0);
    assert_eq!(t.quantity, Winstons::from(0u32));
    assert_eq!(t.reward, r);
    assert!(t.target().is_none());
}

#[test]
fn tx_transfer_style() {
    let c = Client::new().unwrap();
    let (txh, r, q, to) = settings::transfer_transaction();
    let t = c.tx(&txh).unwrap();
    assert_eq!(t.id, txh);
    assert_eq!(t.data.len(), 0);
    assert_eq!(t.quantity, q);
    assert_eq!(t.reward, r);
    assert_eq!(Some(&to), t.target());
}
