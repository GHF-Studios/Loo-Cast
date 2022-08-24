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
            playerInventoryRuntimeData.ItemContainer.OnContentChanged.AddListener(() => { Refresh(); });
        }

        public void Refresh()
        {
            for (int i = 0; i < hotbarSlots.Length; i++)
            {
                hotbarSlots[i].Item = playerInventoryRuntimeData.ItemContainer.GetItem(i);
            }
        }
    }
}
