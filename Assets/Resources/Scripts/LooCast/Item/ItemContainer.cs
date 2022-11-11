using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;

    public class ItemContainer
    {
        protected Dictionary<int, ItemContainerSlot> itemSlots;
        public UnityEvent OnChange
        {
            get
            {
                return onChange;
            }
        }
        public GameObject OriginObject
        {
            get
            {
                return originObject;
            }
        }
        private UnityEvent onChange;
        private GameObject originObject;

        public ItemContainer(int slotCount, GameObject originObject = null)
        {
            if (slotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }

            onChange = new UnityEvent();

            Clear(slotCount);

            if (originObject != null)
            {
                this.originObject = originObject;
            }
        }

        public ItemContainer(Item[] items, GameObject originObject = null)
        {
            if (items == null)
            {
                throw new ArgumentNullException("Items cannot be null!");
            }
            if (items.Length == 0)
            {
                throw new ArgumentOutOfRangeException("Items must have atleast one entry!");
            }
             
            onChange = new UnityEvent();
            
            Clear(itemSlots.Count);
            foreach (Item item in items)
            {
                TryAddItem(item, out Item remainingItem);
            }

            if (originObject != null)
            {
                this.originObject = originObject;
            }
        }

        /// <summary>
        /// Adds and Item to the ItemContainer and returns the remaining Item, if any part of the Item could not be added.
        /// </summary>
        public virtual void TryAddItem(Item item, out Item remainingItem)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }

            if (item is CountableItem)
            {
                TryAddItem((CountableItem)item, out CountableItem remainingCountableItem);
                remainingItem = remainingCountableItem;
                OnChange.Invoke();
                return;
            }
            else if (item is AmountableItem)
            {
                TryAddItem((AmountableItem)item, out AmountableItem remainingAmountableItem);
                remainingItem = remainingAmountableItem;
                OnChange.Invoke();
                return;
            }
            else if (item is UniqueItem)
            {
                TryAddItem((UniqueItem)item, out UniqueItem remainingUniqueItem);
                remainingItem = remainingUniqueItem;
                OnChange.Invoke();
                return;
            }
            else
            {
                throw new NotSupportedException("Unsupported Item Type!");
            }
        }

        protected void TryAddItem(CountableItem countableItem, out CountableItem remainingCountableItem)
        {
            remainingCountableItem = countableItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingCountableItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingCountableItem.UndropItem();
                    }
                    SetItem_Internal(i, remainingCountableItem);
                    remainingCountableItem = null;
                    break;
                }
                else if (itemSlots[i].Equals(remainingCountableItem))
                {
                    CountableItem countableItemSlot = (CountableItem)itemSlots[i].ItemContent;
                    if (!countableItemSlot.IsFull())
                    {
                        int freeCount = countableItemSlot.GetFreeCount();
                        if (freeCount >= remainingCountableItem.Count)
                        {
                            countableItemSlot.Count += remainingCountableItem.Count;
                            remainingCountableItem = null;
                            break;
                        }
                        else
                        {
                            countableItemSlot.Count = countableItemSlot.MaxCount;
                            remainingCountableItem.Count -= freeCount;
                        }
                    }
                }
            }
        }

        protected void TryAddItem(AmountableItem amountableItem, out AmountableItem remainingAmountableItem)
        {
            remainingAmountableItem = amountableItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingAmountableItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingAmountableItem.UndropItem();
                    }
                    SetItem_Internal(i, remainingAmountableItem);
                    remainingAmountableItem = null;
                    break;
                }
                else if (itemSlots[i].Equals(remainingAmountableItem))
                {
                    AmountableItem amountableItemSlot = (AmountableItem)itemSlots[i].ItemContent;
                    if (!amountableItemSlot.IsFull())
                    {
                        float freeAmount = amountableItemSlot.GetFreeAmount();
                        if (freeAmount >= remainingAmountableItem.Amount)
                        {
                            amountableItemSlot.Amount += remainingAmountableItem.Amount;
                            remainingAmountableItem = null;
                            break;
                        }
                        else
                        {
                            amountableItemSlot.Amount = amountableItemSlot.MaxAmount;
                            remainingAmountableItem.Amount -= freeAmount;
                        }
                    }
                }
            }
        }

        protected void TryAddItem(UniqueItem uniqueItem, out UniqueItem remainingUniqueItem)
        {
            remainingUniqueItem = uniqueItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingUniqueItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingUniqueItem.UndropItem();
                    }
                    SetItem_Internal(i, remainingUniqueItem);
                    remainingUniqueItem = null;
                    return;
                }
            }
        }

        /// <summary>
        /// Tries to remove an Item from the ItemContainer
        /// </summary>
        /// <param name="slotID">The ID of the slot to remove the Item from</param>
        /// <returns>The removed Item. Null if there was no item to be removed to begin with.</returns>
        public Item TryRemoveItem(int slotID)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            Item removedItem = GetItem(slotID);
            removedItem.UncontainItem();
            SetItem(slotID, null);
            return removedItem;
        }

        public virtual void SetItem(int slotID, Item item)
        {
            SetItem_Internal(slotID, item);
            OnChange.Invoke();
        }

        // This method exists for the sole purpose of stopping the protected TryAddItem Methods from redundantly invoking the 'OnChange'-Event
        protected void SetItem_Internal(int slotID, Item item)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            item.ContainItem(this);
            itemSlots[slotID].ItemContent = item;
        }

        public Item GetItem(int slotID)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            bool success = itemSlots.TryGetValue(slotID, out ItemContainerSlot slot);
            if (!success)
            {
                throw new Exception($"Unable to get Item at SlotID: {slotID}");
            }
            return slot.ItemContent;
        }

        public Item[] GetItems()
        {
            return itemSlots.GetItems();
        }

        public virtual bool Contains(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            foreach (KeyValuePair<int, ItemContainerSlot> slot in itemSlots)
            {
                if (slot.Value.ItemContent.Equals(item))
                {
                    return true;
                }
            }
            return false;
        }

        public bool Contains(int slotID)
        {
            return itemSlots.ContainsKey(slotID);
        }

        public void Clear(int slotCount)
        {
            itemSlots = new Dictionary<int, ItemContainerSlot>();

            for (int i = 0; i < slotCount; i++)
            {
                RemoveSlot(i);
                AddSlot(i);
            }

            OnChange.Invoke();
        }

        public bool IsValidSlot(int slot)
        {
            return slot < itemSlots.Count && slot >= 0;
        }

        public void AddSlot(int slotID)
        {
            if (itemSlots.ContainsKey(slotID))
            {
                throw new ArgumentException("SlotID is already occupied!");
            }
            itemSlots.Add(slotID, new ItemContainerSlot());
        }

        public void RemoveSlot(int slotID)
        {
            itemSlots.Remove(slotID);
        }

        public bool IsBoundToObject()
        {
            return originObject != null;
        }

        public override string ToString()
        {
            string message = "";
            for (int i = 0; i < itemSlots.Count; i++)
            {
                message += $"Slot {i}:\t";
                if (itemSlots[i] != null)
                {
                    message += $"{itemSlots[i].ItemContent}\n";
                }
                else
                {
                    message += "null\n";
                }
            }
            return message;
        }
    }
}