using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Core;

    public class LoadMainMenuButton : Button
    {
        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            MainManager.LoadMainMenu();
        }
    }
}
