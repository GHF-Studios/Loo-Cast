using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    public class ItemContainer
    {
        protected Item[] itemSlots;
        public UnityEvent<int[]> OnSlotsChanged
        {
            get
            {
                return onSlotsChanged;
            }
        }
        private UnityEvent<int[]> onSlotsChanged;
        protected Func<Item, bool> itemValidator;

        public ItemContainer(int slotCount, Func<Item, bool> itemValidator = null)
        {
            if (slotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }
            itemSlots = new Item[slotCount];
            onSlotsChanged = new UnityEvent<int[]>();
            this.itemValidator = itemValidator;
        }

        public ItemContainer(Item[] items, Func<Item, bool> itemValidator = null)
        {
            if (items == null)
            {
                throw new ArgumentNullException("Items cannot be null!");
            }
            if (items.Length == 0)
            {
                throw new ArgumentOutOfRangeException("Items must have atleast one entry!");
            }
            for (int i = 0; i < items.Length; i++)
            {
                if (itemValidator != null && !itemValidator.Invoke(items[i]))
                {
                    throw new ArgumentException("Items have to be valid, according to the Item Validator!");
                }
            }
            itemSlots = items;
            onSlotsChanged = new UnityEvent<int[]>();
            this.itemValidator = itemValidator;
        }

        public void AddItem(Item item, out Item remainingItem)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }

            if (itemValidator != null && itemValidator.Invoke(item))
            {
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

            remainingItem = item;
        }

        private void AddItem(CountableItem countableItem, out CountableItem remainingCountableItem, out int[] changedSlots)
        {
            remainingCountableItem = countableItem;
            List<int> changedSlotsList = new List<int>();
            for (int i = 0; i < itemSlots.Length; i++)
            {
                if (itemSlots[i] == null)
                {
                    itemSlots[i] = remainingCountableItem;
                    remainingCountableItem = null;

                    changedSlotsList.Add(i);
                    break;
                }
                else if (itemSlots[i].Equals(remainingCountableItem))
                {
                    CountableItem countableItemSlot = (CountableItem)itemSlots[i];
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
            for (int i = 0; i < itemSlots.Length; i++)
            {
                if (itemSlots[i] == null)
                {
                    itemSlots[i] = remainingAmountableItem;
                    remainingAmountableItem = null;

                    changedSlotsList.Add(i);
                    break;
                }
                else if (itemSlots[i].Equals(remainingAmountableItem))
                {
                    AmountableItem amountableItemSlot = (AmountableItem)itemSlots[i];
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
            for (int i = 0; i < itemSlots.Length; i++)
            {
                if (itemSlots[i] == null)
                {
                    itemSlots[i] = uniqueItem;
                    remainingUniqueItem = null;
                    changedSlot = i;
                    return;
                }
            }
            remainingUniqueItem = uniqueItem;
            changedSlot = null;
        }

        public void SetItem(int slot, Item item)
        {
            if (!IsValidSlot(slot))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Length - 1}!");
            }
            itemSlots[slot] = item;
            onSlotsChanged.Invoke(new int[] { slot });
        }

        public Item GetItem(int slot)
        {
            if (!IsValidSlot(slot))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Length - 1}!");
            }
            return itemSlots[slot];
        }

        public Item[] GetItems()
        {
            return itemSlots;
        }

        public bool Contains(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            foreach (Item itemSlot in itemSlots)
            {
                if (itemSlot.Equals(item))
                {
                    return true;
                }
            }
            return false;
        }

        public void Clear()
        {
            itemSlots = new Item[itemSlots.Length];
            int[] changedSlots = new int[itemSlots.Length];
            for (int i = 0; i < changedSlots.Length; i++)
            {
                changedSlots[i] = i;
            }
            onSlotsChanged.Invoke(changedSlots);
        }

        public bool IsValidSlot(int slot)
        {
            return slot < itemSlots.Length && slot >= 0;
        }

        public override string ToString()
        {
            string message = "";
            for (int i = 0; i < itemSlots.Length; i++)
            {
                message += $"Slot {i}:\t";
                if (itemSlots[i] != null)
                {
                    message += $"{itemSlots[i]}\n";
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