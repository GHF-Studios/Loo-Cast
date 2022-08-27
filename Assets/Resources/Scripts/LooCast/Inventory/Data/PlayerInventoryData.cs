using UnityEngine;

namespace LooCast.Inventory.Data
{
    [CreateAssetMenu(fileName = "PlayerInventoryData", menuName = "Data/Inventory/PlayerInventoryData", order = 0)]
    public class PlayerInventoryData : ScriptableObject
    {
        public int SlotCount;
    } 
}
