using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    public class ItemContainerSlot<T> where T : Item
    {
        public T ItemContent
        {
            get
            {
                return itemContent;
            }

            set
            {
                OnItemContentChanged.Invoke();
                itemContent = value;
            }
        }
        public UnityEvent OnItemContentChanged { get; private set; }
        private T itemContent;

        public ItemContainerSlot(T itemContent = null)
        {
            OnItemContentChanged = new UnityEvent();
            this.itemContent = itemContent;
        }
    }
}