using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.MainMenu
{
    public class MainMenuManager : MonoBehaviour
    {
        #region Static Fields
        private static Queue<Action> postInitializationActionQueue;
        #endregion

        #region Properties
        public static MainMenuManager Instance { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            if (Instance != null)
            {
                throw new Exception("Cannot have multiple instances of MainMenuManager!");
            }

            #region Initialization
            Instance = this;
            #endregion

            Debug.Log($"[MainMenuManager] Initialized.");

            if (postInitializationActionQueue != null)
            {
                while (postInitializationActionQueue.Count > 0)
                {
                    Action postInitializationAction = postInitializationActionQueue.Dequeue();
                    postInitializationAction.Invoke();
                }
            }

            Debug.Log($"[MainMenuManager] Post-Initialized.");
        }
        #endregion

        #region Static Methods
        public static void AddPostInitializationAction(Action postInitializationAction)
        {
            if (postInitializationAction == null)
            {
                return;
            }

            if (postInitializationActionQueue == null)
            {
                postInitializationActionQueue = new Queue<Action>();
            }

            postInitializationActionQueue.Enqueue(postInitializationAction);
        }
        #endregion

        #region Methods
        public void Quit()
        {
            Application.Quit();
        }
        #endregion
    }
}
