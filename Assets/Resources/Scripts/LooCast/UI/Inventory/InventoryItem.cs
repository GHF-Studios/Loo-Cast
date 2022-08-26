using System;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Events;
using UnityEngine.EventSystems;

namespace LooCast.UI.Inventory
{
    using LooCast.Item;
    using LooCast.UI.Canvas;
    using LooCast.Util;

    public class InventoryItem : MonoBehaviour, IBeginDragHandler, IEndDragHandler, IDragHandler
    {
        #region Properties
        public UnityEvent OnDestroy
        {
            get
            {
                return onDestroy;
            }
        }
        public Item Item
        {
            get
            {
                return item;
            }

            set
            {
                if (value == null)
                {
                    throw new NullReferenceException("Item cannot be null!");
                }
                item = value;
                Refresh();
            }
        }
        public RectTransform RectTransform
        {
            get
            {
                return rectTransform;

            }
        }
        public InventorySlot CurrentInventorySlot
        {
            get
            {
                return currentInventorySlot;
            }

            set
            {
                if (value == null)
                {
                    throw new NullReferenceException("Current Inventory Slot can not be null!");
                }
                currentInventorySlot = value;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private Image image;
        [SerializeField] private Text quantityValue;
        [SerializeField] private CanvasGroup canvasGroup;
        [SerializeField] private RectTransform rectTransform;

        private UnityEvent onDestroy;
        private Item item;
        private InventorySlot currentInventorySlot;
        private bool revertToCurrentInventorySlot;
        private UnityEngine.Canvas canvas;
        #endregion

        #region Unity Callbacks
        public void OnBeginDrag(PointerEventData eventData)
        {
            canvasGroup.alpha = 0.6f;
            canvasGroup.blocksRaycasts = false;
            revertToCurrentInventorySlot = true;

            rectTransform.anchoredPosition = RectTransformUtil.ScreenToRectPos(Input.mousePosition, rectTransform, canvas);
            rectTransform.SetParent(CurrentInventorySlot.RectTransform.parent);
            rectTransform.SetAsLastSibling();
        }

        public void OnDrag(PointerEventData eventData)
        {
            rectTransform.anchoredPosition += eventData.delta / rectTransform.lossyScale;
        }

        public void OnEndDrag(PointerEventData eventData)
        {
            canvasGroup.alpha = 1.0f;
            canvasGroup.blocksRaycasts = true;

            if (revertToCurrentInventorySlot)
            {
                RevertToCurrentSlot();
            }
        }
        #endregion

        #region Method
        public void Initialize(UnityEngine.Canvas canvas)
        {
            onDestroy = new UnityEvent();
            this.canvas = canvas;
        }

        public void Refresh()
        {
            if (item is CountableItem)
            {
                CountableItem countableItem = (CountableItem)item;
                image.enabled = true;
                image.sprite = countableItem.Sprite;
                quantityValue.enabled = true;
                quantityValue.text = $"{countableItem.Count}";
            }
            else if(item is AmountableItem)
            {
                AmountableItem amountableItem = (AmountableItem)item;
                image.enabled = true;
                image.sprite = amountableItem.Sprite;
                quantityValue.enabled = true;
                quantityValue.text = string.Format("{0:n0}", amountableItem.Amount) + "t";
            }
        }

        public void Destroy()
        {
            Destroy(gameObject);
            onDestroy.Invoke();
        }

        public void DropOntoSlot(InventorySlot inventorySlot)
        {
            if (CurrentInventorySlot != null)
            {
                CurrentInventorySlot.CurrentItem = null;
            }

            CurrentInventorySlot = inventorySlot;
            CurrentInventorySlot.CurrentItem = this;

            revertToCurrentInventorySlot = false;
        }

        public void SwapSlots(InventoryItem otherInventoryItem)
        {
            if (otherInventoryItem.CurrentInventorySlot == null)
            {
                throw new NullReferenceException("Other Inventory Item is not contained in any slot! Drop it into a slot first!");
            }
            if (CurrentInventorySlot == null)
            {
                throw new NullReferenceException("This Inventory Item is not contained in any slot! Drop it into a slot first!");
            }

            InventorySlot thisInventorySlot = CurrentInventorySlot;
            InventorySlot otherInventorySlot = otherInventoryItem.CurrentInventorySlot;

            thisInventorySlot.CurrentItem = otherInventoryItem;
            otherInventorySlot.CurrentItem = this;

            revertToCurrentInventorySlot = false;
        }

        private void RevertToCurrentSlot()
        {
            rectTransform.SetParent(CurrentInventorySlot.RectTransform);
            rectTransform.anchoredPosition = Vector2.zero;
        }
        #endregion
    }
}
