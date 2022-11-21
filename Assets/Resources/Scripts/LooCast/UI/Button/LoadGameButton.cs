using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Core;

    public class LoadGameButton : Button
    {
        [SerializeField] private Text gameNameText;

        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            MainManager.LoadGame(gameNameText.text);
        }
    }
}
