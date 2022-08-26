using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Inventory
{
    using LooCast.Inventory.Data.Runtime;
    using LooCast.Item;

    public class InventoryHotbar : MonoBehaviour
    {
        public InventorySlot CurrentInventorySlot
        {
            get
            {
                return currentInventorySlotCursor.CurrentInventorySlot;
            }

            set
            {
                if (value == null)
                {
                    throw new NullReferenceException("Current Inventory Slot cannot be null!");
                }
                currentInventorySlotCursor.CurrentInventorySlot = value;
            }
        }

        [SerializeField] private PlayerInventoryRuntimeData playerInventoryRuntimeData;
        [SerializeField] private GameObject inventoryItemPrefab;
        [SerializeField] private InventorySlot[] inventorySlots;
        [SerializeField] private InventorySlotCursor currentInventorySlotCursor;

        private void Start()
        {
            playerInventoryRuntimeData.ItemContainer.OnSlotsChanged.AddListener((slots) => { RefreshSlots(slots); });

            for (int i = 0; i < inventorySlots.Length; i++)
            {
                inventorySlots[i].Initialize(i, playerInventoryRuntimeData.ItemContainer);
            }
        }

        private void Update()
        {
            if (Input.GetKeyDown(KeyCode.Alpha1))
            {
                CurrentInventorySlot = inventorySlots[0];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha2))
            {
                CurrentInventorySlot = inventorySlots[1];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha3))
            {
                CurrentInventorySlot = inventorySlots[2];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha4))
            {
                CurrentInventorySlot = inventorySlots[3];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha5))
            {
                CurrentInventorySlot = inventorySlots[4];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha6))
            {
                CurrentInventorySlot = inventorySlots[5];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha7))
            {
                CurrentInventorySlot = inventorySlots[6];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha8))
            {
                CurrentInventorySlot = inventorySlots[7];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha9))
            {
                CurrentInventorySlot = inventorySlots[8];
            }
            else if (Input.GetKeyDown(KeyCode.Alpha0))
            {
                CurrentInventorySlot = inventorySlots[9];
            }

            if (Input.GetKeyDown(KeyCode.Q))
            {
                if (CurrentInventorySlot.CurrentItem != null)
                {
                    CurrentInventorySlot.CurrentItem.Item.SpawnItem((Vector2)Camera.main.ScreenToWorldPoint(Input.mousePosition));
                    CurrentInventorySlot.ItemContainer.SetItem(CurrentInventorySlot.SlotID, null);
                    CurrentInventorySlot.CurrentItem.Destroy();
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
