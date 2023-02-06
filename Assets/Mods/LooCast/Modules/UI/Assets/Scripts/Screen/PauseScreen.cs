using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using LooCast.UI.Button;
using LooCast.UI.Canvas;

namespace LooCast.UI.Screen
{
    public class PauseScreen : Screen
    {
        private void Start()
        {
            isInitiallyVisible = false;
            isHideable = true;
            Initialize();
        }

        public override void SetVisibility(bool show)
        {
            base.SetVisibility(show);
        }
    } 
}
