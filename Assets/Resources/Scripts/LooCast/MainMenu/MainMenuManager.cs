using System.Collections;
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
        [SerializeField] private LoadingScreen LoadingScreen;
        #endregion

        #region Methods
        private void Awake()
        {
            if (Instance != null && Instance != this)
            {
                Destroy(gameObject);
            }
            else
            {
                Instance = this;
            }
        }

        public void Quit()
        {
            if (Instance == null)
            {
                return;
            }
            Application.Quit();
        }

        public void LoadScene(string sceneIndex)
        {
            if (Instance == null)
            {
                return;
            }
            Instance.StartCoroutine(Instance.LoadingScreen.LoadSceneAsynchronously(sceneIndex));
        }
        #endregion
    }
}
