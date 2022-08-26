using System;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Events;
using UnityEngine.EventSystems;

namespace LooCast.UI.Inventory
{
    using LooCast.Item;
    using LooCast.UI.Canvas;

    public class InventoryItem : MonoBehaviour, IBeginDragHandler, IEndDragHandler, IDragHandler
    {
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
        
        [SerializeField] private Image image;
        [SerializeField] private Text quantityValue;
        [SerializeField] private CanvasGroup canvasGroup;
        [SerializeField] private RectTransform rectTransform;

        private UnityEvent onDestroy;
        private Item item;
        private InventorySlot currentInventorySlot;
        private bool droppedOntoOtherSlot;

        #region Unity Callbacks
        private void Start()
        {
            onDestroy = new UnityEvent();
        }

        public void OnBeginDrag(PointerEventData eventData)
        {
            canvasGroup.alpha = 0.6f;
            canvasGroup.blocksRaycasts = false;
            droppedOntoOtherSlot = false;

            rectTransform.SetParent(currentInventorySlot.RectTransform.parent);
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

            if (!droppedOntoOtherSlot)
            {
                RevertToCurrentSlot();
            }
        }
        #endregion

        #region Method
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
            if (currentInventorySlot != null)
            {
                currentInventorySlot.CurrentItem = null;
            }
            currentInventorySlot = inventorySlot;
            currentInventorySlot.CurrentItem = this;

            rectTransform.SetParent(inventorySlot.RectTransform);
            rectTransform.anchoredPosition = Vector2.zero;

            droppedOntoOtherSlot = true;
        }

        public void RevertToCurrentSlot()
        {
            rectTransform.SetParent(currentInventorySlot.RectTransform);
            rectTransform.anchoredPosition = Vector2.zero;
        }
        #endregion
    }
}
