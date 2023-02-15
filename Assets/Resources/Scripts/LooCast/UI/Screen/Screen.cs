using CSSystem = System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Screen
{
    using LooCast.System;
    using LooCast.Core;
    using LooCast.Game;
    using LooCast.UI.Canvas;

    public abstract class Screen : ExtendedMonoBehaviour
    {
        protected bool isHideable;
        protected bool isInitiallyVisible;
        protected List<GameObject> hideableObjects;
        public InterfaceCanvas Canvas;
        public int priority = -1;

        protected void Initialize()
        {
            IsVisible = isInitiallyVisible;

            if (!isHideable)
            {
                if (!isInitiallyVisible)
                {
                    throw new CSSystem.Exception("Screen is initialized as initially invisible and unhideable!");
                }
            }

            if (isInitiallyVisible)
            {
                transform.SetAsLastSibling();
                if (Canvas.screenStack.Count > 0)
                {
                    throw new CSSystem.Exception("More than one screen initially visible!");
                }
                Canvas.screenStack.Push(this);
            }

            if (isHideable)
            {
                hideableObjects = new List<GameObject>();
                for (int i = 0; i < transform.childCount; i++)
                {
                    hideableObjects.Add(transform.GetChild(i).gameObject);
                }
            }
        }

        public virtual void Refresh()
        {

        }

        public virtual void SetVisibility(bool show)
        {
            if (!show)
            {
                if (!isHideable)
                {
                    return;
                }

                if (Canvas.screenStack.Peek().Equals(this))
                {
                    Canvas.screenStack.Pop();
                    if (Canvas.screenStack.Count == 0)
                    {
                        GameManager.ResumeGame();
                    }
                }
                else
                {
                    return;
                }
            }

            if (show)
            {
                if (priority == -1)
                {
                    throw new CSSystem.Exception("Priority is not set!");
                }

                foreach (Screen screen in Canvas.screenStack)
                {
                    if (priority < screen.priority)
                    {
                        return;
                    }
                }
                Canvas.screenStack.Push(this);
                transform.SetAsLastSibling();
                GameManager.PauseGame();
            }

            IsVisible = show;
            foreach (GameObject obj in hideableObjects)
            {
                obj.SetActive(show);
            }

            if (IsVisible)
            {
                Refresh();
            }
        }

        public virtual Stack<Screen> GetScreenStack()
        {
            return Canvas.screenStack;
        }

        public virtual bool GetVisibility()
        {
            return IsVisible;
        }

        public virtual void ToggleVisibility()
        {
            SetVisibility(!IsVisible);
        }
    } 
}
