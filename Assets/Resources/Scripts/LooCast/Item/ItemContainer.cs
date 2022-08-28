using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;

    public class ItemContainer<T> where T : Item
    {
        protected Dictionary<int, ItemContainerSlot<T>> itemSlots;
        public UnityEvent<int[]> OnSlotsChanged
        {
            get
            {
                return onSlotsChanged;
            }
        }
        private UnityEvent<int[]> onSlotsChanged;

        public ItemContainer(int slotCount)
        {
            if (slotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }

            onSlotsChanged = new UnityEvent<int[]>();

            Clear(slotCount);
        }

        public ItemContainer(T[] items)
        {
            if (items == null)
            {
                throw new ArgumentNullException("Items cannot be null!");
            }
            if (items.Length == 0)
            {
                throw new ArgumentOutOfRangeException("Items must have atleast one entry!");
            }
             
            onSlotsChanged = new UnityEvent<int[]>();
            
            Clear(itemSlots.Count);
            foreach (T item in items)
            {
                AddItem(item, out Item remainingItem);
            }
        }

        public void AddItem(Item item, out Item remainingItem)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }

            if (item is CountableItem)
            {
                AddItem((CountableItem)item, out CountableItem remainingCountableItem, out int[] changedSlots);
                remainingItem = remainingCountableItem;
                onSlotsChanged.Invoke(changedSlots);
                return;
            }
            else if (item is AmountableItem)
            {
                AddItem((AmountableItem)item, out AmountableItem remainingAmountableItem, out int[] changedSlots);
                remainingItem = remainingAmountableItem;
                onSlotsChanged.Invoke(changedSlots);
                return;
            }
            else if (item is UniqueItem)
            {
                AddItem((UniqueItem)item, out UniqueItem remainingUniqueItem, out int? changedSlot);
                remainingItem = remainingUniqueItem;
                onSlotsChanged.Invoke(changedSlot != null ? new int[] { (int)changedSlot } : Array.Empty<int>());
                return;
            }
            else
            {
                throw new NotSupportedException("Unsupported Item Type!");
            }
        }

        private void AddItem(CountableItem countableItem, out CountableItem remainingCountableItem, out int[] changedSlots)
        {
            remainingCountableItem = countableItem;
            List<int> changedSlotsList = new List<int>();
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    itemSlots[i].ItemContent = (T)(Item)remainingCountableItem;
                    remainingCountableItem = null;

                    changedSlotsList.Add(i);
                    break;
                }
                else if (itemSlots[i].Equals(remainingCountableItem))
                {
                    CountableItem countableItemSlot = (CountableItem)(Item)itemSlots[i].ItemContent;
                    if (!countableItemSlot.IsFull())
                    {
                        int freeCount = countableItemSlot.GetFreeCount();
                        if (freeCount >= remainingCountableItem.Count)
                        {
                            countableItemSlot.Count += remainingCountableItem.Count;
                            remainingCountableItem = null;

                            changedSlotsList.Add(i);
                            break;
                        }
                        else
                        {
                            countableItemSlot.Count = countableItemSlot.MaxCount;
                            remainingCountableItem.Count -= freeCount;

                            changedSlotsList.Add(i);
                        }
                    }
                }
            }
            changedSlots = changedSlotsList.ToArray();
        }

        private void AddItem(AmountableItem amountableItem, out AmountableItem remainingAmountableItem, out int[] changedSlots)
        {
            remainingAmountableItem = amountableItem;
            List<int> changedSlotsList = new List<int>();
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    itemSlots[i].ItemContent = (T)(Item)remainingAmountableItem;
                    remainingAmountableItem = null;

                    changedSlotsList.Add(i);
                    break;
                }
                else if (itemSlots[i].Equals(remainingAmountableItem))
                {
                    AmountableItem amountableItemSlot = (AmountableItem)(Item)itemSlots[i].ItemContent;
                    if (!amountableItemSlot.IsFull())
                    {
                        float freeAmount = amountableItemSlot.GetFreeAmount();
                        if (freeAmount >= remainingAmountableItem.Amount)
                        {
                            amountableItemSlot.Amount += remainingAmountableItem.Amount;
                            remainingAmountableItem = null;

                            changedSlotsList.Add(i);
                            break;
                        }
                        else
                        {
                            amountableItemSlot.Amount = amountableItemSlot.MaxAmount;
                            remainingAmountableItem.Amount -= freeAmount;

                            changedSlotsList.Add(i);
                        }
                    }
                }
            }
            changedSlots = changedSlotsList.ToArray();
        }

        private void AddItem(UniqueItem uniqueItem, out UniqueItem remainingUniqueItem, out int? changedSlot)
        {
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    itemSlots[i].ItemContent = (T)(Item)uniqueItem;
                    remainingUniqueItem = null;
                    changedSlot = i;
                    return;
                }
            }
            remainingUniqueItem = uniqueItem;
            changedSlot = null;
        }

        public void SetItem(int slotID, T item)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            itemSlots[slotID].ItemContent = item;
            onSlotsChanged.Invoke(new int[] { slotID });
        }

        public Item GetItem(int slotID)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            bool success = itemSlots.TryGetValue(slotID, out ItemContainerSlot<T> slot);
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

        public bool Contains(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            foreach (KeyValuePair<int, ItemContainerSlot<T>> slot in itemSlots)
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
            itemSlots = new Dictionary<int, ItemContainerSlot<T>>();

            int[] changedSlots = new int[slotCount];
            for (int i = 0; i < slotCount; i++)
            {
                RemoveSlot(i);
                AddSlot(i);
                changedSlots[i] = i;
            }

            onSlotsChanged.Invoke(changedSlots);
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
            itemSlots.Add(slotID, new ItemContainerSlot<T>());
        }

        public void RemoveSlot(int slotID)
        {
            itemSlots.Remove(slotID);
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