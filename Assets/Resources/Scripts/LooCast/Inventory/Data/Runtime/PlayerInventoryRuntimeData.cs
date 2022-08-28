using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Inventory.Data.Runtime
{
    using LooCast.Item;
    
    [CreateAssetMenu(fileName = "PlayerInventoryRuntimeData", menuName = "Data/Inventory/Runtime/PlayerInventoryRuntimeData", order = 0)]
    public sealed class PlayerInventoryRuntimeData : ScriptableObject
    {
        public ItemContainer<Item> Hotbar { get; private set; }

        public void Initialize(PlayerInventoryData data)
        {
            Hotbar = new ItemContainer<Item>(data.SlotCount.Value);
        }
        
    }
}
