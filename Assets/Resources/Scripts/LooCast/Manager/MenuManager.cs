using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Manager
{
    using LooCast.UI.Screen;
    
    public class MenuManager : MonoBehaviour
    {
        #region Properties
        public static MenuManager Instance { get; private set; }
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
            Application.Quit();
        }

        public void LoadScene(string sceneIndex)
        {
            StartCoroutine(LoadingScreen.LoadSceneAsynchronously(sceneIndex));
        }
        #endregion
    }
}
