using UnityEngine;

namespace LooCast.UI.Button
{
    using Core;

    public class CreateGameButton : Button
    {
        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            MainManager.CreateNewGame("New Game");
        }
    }
}
