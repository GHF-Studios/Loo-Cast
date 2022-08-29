using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Inventory
{
    using Data;
    using LooCast.Inventory.Data.Runtime;
    using LooCast.Item;
    using LooCast.Event;

    public class InventoryHotbar : MonoBehaviour
    {
        #region Data
        [SerializeField] private InventoryHotbarData data;
        #endregion

        #region Properties
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
        #endregion

        #region Fields
        [SerializeField] private PlayerInventoryRuntimeData playerInventoryRuntimeData;
        [SerializeField] private GameObject inventoryItemPrefab;
        [SerializeField] private InventorySlot[] inventorySlots;
        [SerializeField] private InventorySlotCursor currentInventorySlotCursor;
        [SerializeField] private UnityEngine.Canvas canvas;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            for (int i = 0; i < inventorySlots.Length; i++)
            {
                inventorySlots[i].Initialize(i, playerInventoryRuntimeData.Hotbar, canvas);
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

            float scaledDeltaMouseScroll = (Input.mouseScrollDelta * data.MouseScrollScale).y;
            int currentSlotID = CurrentInventorySlot.SlotID;
            if (scaledDeltaMouseScroll < -0.1f)
            {
                currentSlotID++;
            }
            else if (scaledDeltaMouseScroll > 0.1f)
            {
                currentSlotID--;
            }

            if (currentSlotID >= inventorySlots.Length)
            {
                currentSlotID = 0;
            }
            else if (currentSlotID < 0)
            {
                currentSlotID = inventorySlots.Length - 1;
            }
            CurrentInventorySlot = inventorySlots[currentSlotID];

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
        #endregion

        #region Methods
        public void RefreshSlots()
        {
            foreach (InventorySlot inventorySlot in inventorySlots)
            {
                Item item = playerInventoryRuntimeData.Hotbar.GetItem(inventorySlot.SlotID);
                if (item == null)
                {
                    if (inventorySlot.CurrentItem != null)
                    {
                        inventorySlot.CurrentItem.Destroy();
                    }
                }
                else
                {
                    if (inventorySlot.CurrentItem == null)
                    {
                        GameObject inventoryItemObject = Instantiate(inventoryItemPrefab, inventorySlot.transform);
                        InventoryItem inventoryItem = inventoryItemObject.GetComponent<InventoryItem>();
                        inventoryItem.Initialize(canvas);
                        inventoryItem.Item = playerInventoryRuntimeData.Hotbar.GetItem(inventorySlot.SlotID);
                        inventoryItem.DropOntoSlot(inventorySlot);
                    }
                    else
                    {
                        inventorySlot.CurrentItem.Item = playerInventoryRuntimeData.Hotbar.GetItem(inventorySlot.SlotID);
                    }
                }
            }
        }
        #endregion
    }
}
