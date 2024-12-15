using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using Game;

    public class LoadGameButton : Button
    {
        [SerializeField] private Text gameNameText;

        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            GameManager.Instance.LoadGame(gameNameText.text);
        }
    }
}
