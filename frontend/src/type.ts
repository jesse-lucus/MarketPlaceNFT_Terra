export enum ExecuteMsg {
  SetPaused = 'set_paused',
  CreateOrder = 'create_order',
  UpdateOrder = 'update_order',
  CreateBid = 'create_bid',
  CancelOrder = 'cancel_order',
  CancelBid = 'cancel_bid',
  SafeExecuteOrder = 'safe_execute_order',
  AcceptBid ='accept_bid'
}
