import {contractInt,initialize, create_account, deposit, balance} from './test_contract.ts'


initialize().catch((e)=>{
   console.log(e)
})


create_account().catch((e)=>{console.log(e)}).then(v => console.log(v))

deposit().catch((e)=>{console.log(e)}).then(v => console.log(v))

balance().catch((e)=>{console.log(e)}).then(v => console.log(v))




// let x = client.init(&admin_add);
// let y = client.create_account(&person, &10);
// let z = client.deposit(&person, &10);
// let a = client.accounts(&admin_add);
// let _d = client.widraw(&person, &5);
// let b = client.myaccount_balance(&person);
// let c = client.close_account(&person, &admin_add);