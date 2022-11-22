using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.MainMenu
{
    using LooCast.UI.Screen;
    
    public class MainMenuManager : MonoBehaviour
    {
        #region Properties
        public static MainMenuManager Instance { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Methods
        public void Initialize()
        {
            if (Instance != null)
            {
                throw new Exception("Cannot have multiple instances of MainMenuManager!");
            }

            #region Initialization
            Instance = this;
            #endregion

            Debug.Log($"[MainMenuManager] Initialized.");
        }

        public void Quit()
        {
            Application.Quit();
        }
        #endregion
    }
}
