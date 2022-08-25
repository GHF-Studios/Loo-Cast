using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Hotbar
{
    using LooCast.Item;
    using LooCast.Inventory.Data.Runtime;

    public class Hotbar : MonoBehaviour
    {
        [SerializeField] private PlayerInventoryRuntimeData playerInventoryRuntimeData;
        [SerializeField] private HotbarSlot[] hotbarSlots;

        private void Start()
        {
            playerInventoryRuntimeData.ItemContainer.OnContentChanged.AddListener((slots) => { RefreshSlots(slots); });
        }

        public void RefreshSlots(int[] slots)
        {
            foreach (int slot in slots)
            {
                hotbarSlots[slot].Item = playerInventoryRuntimeData.ItemContainer.GetItem(slot);
            }
        }
    }
}
