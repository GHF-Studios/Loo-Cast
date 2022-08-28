using UnityEngine;

namespace LooCast.Inventory.Data
{
    using LooCast.Item.Data;
    using LooCast.Data;

    [CreateAssetMenu(fileName = "PlayerStationInventoryData", menuName = "Data/Inventory/PlayerStationInventoryData", order = 0)]
    public class PlayerStationInventoryData : ScriptableObject
    {
        public IntDataReference SlotCount;
    } 
}
