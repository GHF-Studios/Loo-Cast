using UnityEngine;

namespace LooCast.Inventory.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllyStationInventoryData", menuName = "Data/Inventory/AllyStationInventoryData", order = 0)]
    public class AllyStationInventoryData : ScriptableObject
    {
        public IntDataReference SlotCount;
    } 
}
