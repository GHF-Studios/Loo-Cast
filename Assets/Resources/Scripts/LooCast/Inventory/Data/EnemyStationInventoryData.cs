using UnityEngine;

namespace LooCast.Inventory.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyStationInventoryData", menuName = "Data/Inventory/EnemyStationInventoryData", order = 0)]
    public class EnemyStationInventoryData : ScriptableObject
    {
        public IntDataReference SlotCount;
    } 
}
