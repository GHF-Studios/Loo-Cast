using UnityEngine;

namespace LooCast.UI.Inventory.Data
{
    [CreateAssetMenu(fileName = "InventoryHotbarData", menuName = "Data/UI/Inventory/InventoryHotbarData", order = 0)]
    public sealed class InventoryHotbarData : ScriptableObject
    {
        public float MouseScrollScale;
    }
}