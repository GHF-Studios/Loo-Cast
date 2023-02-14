using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Scene;

    public class LoadMainMenuButton : Button
    {
        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            SceneManager.Instance.LoadScene(SceneManager.SceneType.MainMenu);
        }
    }
}
