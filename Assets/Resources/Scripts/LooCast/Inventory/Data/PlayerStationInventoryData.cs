using UnityEngine;

namespace LooCast.Inventory.Data
{
    using LooCast.Item.Data;

    [CreateAssetMenu(fileName = "PlayerStationInventoryData", menuName = "Data/Inventory/PlayerStationInventoryData", order = 0)]
    public class PlayerStationInventoryData : ScriptableObject
    {
        public WeaponItemData[] DefaultWeapons; 
    } 
}
