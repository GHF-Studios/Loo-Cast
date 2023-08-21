using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Events;
using UnityEngine.EventSystems;
using LooCast.Sound;

namespace LooCast.UI.Button
{
    public abstract class Button : MonoBehaviour, IPointerEnterHandler, IPointerExitHandler
    {
        public UnityEngine.UI.Button UnityButton
        {
            get
            {
                return button;
            }
        }
        [SerializeField] protected UnityEngine.UI.Button button;
        protected MenuSoundHandler soundHandler;

        public void Initialize()
        {
            if (button == null)
            {
                button = GetComponent<UnityEngine.UI.Button>();
                if (button == null)
                {
                    throw new MissingComponentException("A UnityEngine.Button Component is required, but could not be found!");
                }
            }
            
            button.onClick.AddListener(OnClick);
            soundHandler = GameObject.FindObjectOfType<MenuSoundHandler>();
        }

        public virtual void Select()
        {
            button.Select();
        }

        public abstract void OnClick();

        public virtual void OnHoverStart()
        {

        }

        public virtual void OnHoverStop()
        {

        }

        public void OnPointerEnter(PointerEventData eventData)
        {
            OnHoverStart();
        }

        public void OnPointerExit(PointerEventData eventData)
        {
            OnHoverStop();
        }
    } 
}
