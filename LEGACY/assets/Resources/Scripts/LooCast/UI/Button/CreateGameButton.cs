using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Game;

    public class CreateGameButton : Button
    {
        [SerializeField] private Text gameNameText;

        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            string newGameName;
            if (gameNameText.text == "")
            {
                newGameName = "New Game";
            }
            else
            {
                newGameName = gameNameText.text;
            }

            GameManager.Instance.CreateNewGame(newGameName);
        }
    }
}
