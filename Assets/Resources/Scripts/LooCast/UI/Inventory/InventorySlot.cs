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
                }
            }
        }
        public int SlotID { get; private set; }
        public ItemContainer ItemContainer { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private Image image;

        private InventoryItem currentItem;
        #endregion

        #region Unity Callbacks
        public void OnDrop(PointerEventData eventData)
        {
            InventoryItem inventoryItem = eventData.pointerDrag.GetComponent<InventoryItem>();
            if (inventoryItem != null)
            {
                inventoryItem.DropOntoSlot(this);
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
        public void Initialize(int slotID, ItemContainer itemContainer)
        {
            SlotID = slotID;
            ItemContainer = itemContainer;
        }
        #endregion
    }
}
