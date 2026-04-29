using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using LooCast.UI.Tab;
using LooCast.UI.Canvas;

namespace LooCast.UI.Screen
{
    public class StatsScreen : Screen
    {
        private void Start()
        {
            isInitiallyVisible = false;
            isHideable = true;
            Initialize();
        }
    } 
}
