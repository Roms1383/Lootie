module Lootie

public static func HasLootableItems(self: wref<ScriptedPuppet>) -> Bool {
    if !IsDefined(self) { return false; }
    
    return GameInstance.GetTransactionSystem(self.GetGame())
    .GetNumItems(self) > 0;
}