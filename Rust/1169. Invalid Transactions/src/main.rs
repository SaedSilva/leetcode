fn main() {
    let transactions = vec![
        "alice,20,800,mtv".into(),
        "alice,50,100,mtv".into(),
        "alice,51,100,frankfurt".into(),
    ];

    println!("{:?}", invalid_transactions(transactions));
}

pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    let mut invalid_transactions: Vec<bool> = transactions.iter().map(|_| false).collect();
    transactions.iter().enumerate().for_each(|(i, x)| {
        let t = Transaction::from_csv(x);
        if t.money > 1000 {
            invalid_transactions[i] = true;
        }
        for j in i+1..transactions.len() {
            let t2 = Transaction::from_csv(&transactions[j]);
                if t.time.abs_diff(t2.time) <= 60 && t.name == t2.name && t.city != t2.city {
                    invalid_transactions[i] = true;
                    invalid_transactions[j] = true;
                }
        }
    });
    transactions
        .into_iter()
        .enumerate()
        .filter(|(index, _)| invalid_transactions[*index])
        .map(|x| x.1)
        .collect()
}

pub struct Transaction {
    name: String,
    time: u16,
    money: u16,
    city: String,
}

impl Transaction {
    pub fn from_csv(csv: &str) -> Self {
        let splited: Vec<&str> = csv.split(",").collect();
        Self {
            name: splited.get(0).unwrap().to_string(),
            time: splited.get(1).unwrap().parse().unwrap(),
            money: splited.get(2).unwrap().parse().unwrap(),
            city: splited.get(3).unwrap().to_string(),
        }
    }

    pub fn into_csv(&self) -> String {
        let result = vec![
            self.name.clone(),
            self.time.to_string(),
            self.money.to_string(),
            self.city.clone(),
        ]
        .join(",");
        result
    }
}
