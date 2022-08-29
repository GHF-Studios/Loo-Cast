using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Inventory.Data.Runtime
{
    using LooCast.Item;
    using LooCast.Event;

    [CreateAssetMenu(fileName = "PlayerInventoryRuntimeData", menuName = "Data/Inventory/Runtime/PlayerInventoryRuntimeData", order = 0)]
    public sealed class PlayerInventoryRuntimeData : ScriptableObject
    {
        #region Events
        [SerializeField] private Event onPlayerInventoryChange;
        #endregion

        #region Properties
        public ItemContainer<Item> Hotbar { get; private set; }
        #endregion

        #region Methods
        public void Initialize(PlayerInventoryData data)
        {
            Hotbar = new ItemContainer<Item>(data.SlotCount.Value);
            Hotbar.OnChange.AddListener(() => { onPlayerInventoryChange.Raise(); });
        }
        #endregion
    }
}
