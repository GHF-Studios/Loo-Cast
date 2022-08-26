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
        [SerializeField] private InventorySlot currentInventorySlot;
        [SerializeField] private GameObject inventoryItemPrefab;

        private void Start()
        {
            playerInventoryRuntimeData.ItemContainer.OnSlotsChanged.AddListener((slots) => { RefreshSlots(slots); });

            for (int i = 0; i < inventorySlots.Length; i++)
            {
                inventorySlots[i].Initialize(i, playerInventoryRuntimeData.ItemContainer);
            }
            currentInventorySlot = inventorySlots[0];

        }

        private void Update()
        {
            if (Input.GetKeyDown(KeyCode.Alpha1))
            {
                currentInventorySlot = inventorySlots[0];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha2))
            {
                currentInventorySlot = inventorySlots[1];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha3))
            {
                currentInventorySlot = inventorySlots[2];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha4))
            {
                currentInventorySlot = inventorySlots[3];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha5))
            {
                currentInventorySlot = inventorySlots[4];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha6))
            {
                currentInventorySlot = inventorySlots[5];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha7))
            {
                currentInventorySlot = inventorySlots[6];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha8))
            {
                currentInventorySlot = inventorySlots[7];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha9))
            {
                currentInventorySlot = inventorySlots[8];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha0))
            {
                currentInventorySlot = inventorySlots[9];
            }

            if (Input.GetKeyDown(KeyCode.Q))
            {
                if (currentInventorySlot.CurrentItem != null)
                {
                    currentInventorySlot.CurrentItem.Item.SpawnItem((Vector2)Camera.main.ScreenToWorldPoint(Input.mousePosition));
                    currentInventorySlot.ItemContainer.SetItem(currentInventorySlot.SlotID, null);
                    currentInventorySlot.CurrentItem.Destroy();
                }
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
