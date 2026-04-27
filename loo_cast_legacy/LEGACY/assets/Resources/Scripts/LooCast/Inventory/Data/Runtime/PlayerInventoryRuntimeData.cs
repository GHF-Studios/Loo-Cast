using UnityEngine;

namespace LooCast.Inventory.Data.Runtime
{
    using LooCast.Item;
    using LooCast.Event;
    using LooCast.Player;

    [CreateAssetMenu(fileName = "PlayerInventoryRuntimeData", menuName = "Data/Inventory/Runtime/PlayerInventoryRuntimeData", order = 0)]
    public sealed class PlayerInventoryRuntimeData : ScriptableObject
    {
        #region Events
        [SerializeField] private Event onPlayerInventoryInitialize;
        [SerializeField] private Event onPlayerInventoryChange;
        #endregion

        #region Properties
        public ItemContainer Hotbar { get; private set; }
        #endregion

        #region Methods
        public void Initialize(PlayerInventoryData data, Player player)
        {
            Hotbar = new ItemContainer(data.SlotCount.Value, player.gameObject);
            Hotbar.OnChange.AddListener(() => { onPlayerInventoryChange.Raise(); });
            onPlayerInventoryInitialize.Raise();
        }
        #endregion
    }
}
