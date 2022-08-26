using System;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.UI;

namespace LooCast.UI.Inventory
{
    using LooCast.Item;

    public class InventorySlot : MonoBehaviour, IDropHandler, IPointerEnterHandler, IPointerExitHandler
    {
        #region Properties
        public InventoryItem CurrentItem
        {
            get
            {
                return currentItem;
            }

            set
            {
                currentItem = value;
                if (currentItem == null)
                {
                    ItemContainer.SetItem(SlotID, null);
                }
                else
                {
                    ItemContainer.SetItem(SlotID, currentItem.Item);
                    currentItem.CurrentInventorySlot = this;

                    currentItem.RectTransform.SetParent(RectTransform);
                    currentItem.RectTransform.anchoredPosition = Vector2.zero;
                }
            }
        }
        public int SlotID { get; private set; }
        public ItemContainer ItemContainer { get; private set; }
        public RectTransform RectTransform
        {
            get
            {
                return rectTransform;

            }
        }
        #endregion

        #region Fields
        [SerializeField] private Image image;
        [SerializeField] private RectTransform rectTransform;

        private InventoryItem currentItem;
        private UnityEngine.Canvas canvas;
        #endregion

        #region Unity Callbacks
        public void OnDrop(PointerEventData eventData)
        {
            InventoryItem inventoryItem = eventData.pointerDrag.GetComponent<InventoryItem>();
            if (inventoryItem != null)
            {
                if (currentItem == null)
                {
                    inventoryItem.DropOntoSlot(this); 
                }
                else
                {
                    inventoryItem.SwapSlots(CurrentItem);
                }
            }

        }

        public void OnPointerEnter(PointerEventData eventData)
        {
            Color color = image.color;
            color.a = 0.25f;
            image.color = color;
        }

        public void OnPointerExit(PointerEventData eventData)
        {
            Color color = image.color;
            color.a = 0.0f;
            image.color = color;
        }
        #endregion

        #region Methods
        public void Initialize(int slotID, ItemContainer itemContainer, UnityEngine.Canvas canvas)
        {
            SlotID = slotID;
            ItemContainer = itemContainer;
            this.canvas = canvas;
        }
        #endregion
    }
}
