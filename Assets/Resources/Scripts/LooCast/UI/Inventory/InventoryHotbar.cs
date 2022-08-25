using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Inventory
{
    using LooCast.Inventory.Data.Runtime;
    using LooCast.Item;

    public class InventoryHotbar : MonoBehaviour
    {
        [SerializeField] private PlayerInventoryRuntimeData playerInventoryRuntimeData;
        [SerializeField] private InventorySlot[] inventorySlots;
        [SerializeField] private GameObject inventoryItemPrefab;

        private void Start()
        {
            playerInventoryRuntimeData.ItemContainer.OnSlotsChanged.AddListener((slots) => { RefreshSlots(slots); });

            for (int i = 0; i < inventorySlots.Length; i++)
            {
                inventorySlots[i].Initialize(i, playerInventoryRuntimeData.ItemContainer);
            }
        }

        public void RefreshSlots(int[] slots)
        {
            foreach (int slot in slots)
            {
                Item item = playerInventoryRuntimeData.ItemContainer.GetItem(slot);
                if (item == null)
                {
                    if (inventorySlots[slot].CurrentItem != null)
                    {
                        inventorySlots[slot].CurrentItem.Destroy();
                    }
                }
                else
                {
                    if (inventorySlots[slot].CurrentItem == null)
                    {
                        GameObject inventoryItemObject = Instantiate(inventoryItemPrefab, inventorySlots[slot].transform);
                        InventoryItem inventoryItem = inventoryItemObject.GetComponent<InventoryItem>();
                        inventoryItem.Item = playerInventoryRuntimeData.ItemContainer.GetItem(slot);
                        inventoryItem.DropOntoSlot(inventorySlots[slot]);
                    }
                    else
                    {
                        inventorySlots[slot].CurrentItem.Item = playerInventoryRuntimeData.ItemContainer.GetItem(slot);
                    }
                }
            }
        }
    }
}
