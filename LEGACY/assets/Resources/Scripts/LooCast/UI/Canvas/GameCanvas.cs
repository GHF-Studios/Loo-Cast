using UnityEngine;
using LooCast.UI.Screen;
using LooCast.Game;

namespace LooCast.UI.Canvas
{
    public class GameCanvas : ScreenSpaceCameraCanvas
    {
        public PauseScreen PauseScreen;

        private void Update()
        {
            if (Input.GetKeyDown(KeyCode.Escape))
            {
                if (screenStack.Count == 0)
                {
                    PauseScreen.SetVisibility(true);
                    GameManager.PauseGame();
                }
                else
                {
                    screenStack.Peek().SetVisibility(false);
                }
            }
        }
    }
}
