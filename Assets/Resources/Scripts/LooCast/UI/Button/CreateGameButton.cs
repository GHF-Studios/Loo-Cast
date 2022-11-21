using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Core;

    public class CreateGameButton : Button
    {
        [SerializeField] private Text gameNameText;

        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            if (gameNameText.text == "")
            {
                MainManager.CreateNewGame("New Game");
            }
            else
            {
                MainManager.CreateNewGame(gameNameText.text);
            }
        }
    }
}
