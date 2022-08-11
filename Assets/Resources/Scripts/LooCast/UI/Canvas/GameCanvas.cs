using UnityEngine;
using LooCast.UI.Screen;
using LooCast.Manager;

namespace LooCast.UI.Canvas
{
    using Bar;

    public class GameCanvas : ScreenSpaceCameraCanvas
    {
        public PauseScreen pauseScreen { get; private set; }

        public override void Initialize()
        {
            base.Initialize();
            pauseScreen = GetComponentInChildren<PauseScreen>();
        }

        private void Update()
        {
            if (Input.GetKeyDown(KeyCode.Escape))
            {
                if (screenStack.Count == 0)
                {
                    pauseScreen.SetVisibility(true);
                    GameSceneManager.Pause();
                }
                else
                {
                    screenStack.Peek().SetVisibility(false);
                }
            }
        }
    }
}
