using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Canvas
{
    public class InterfaceCanvas : Canvas
    {
        public Stack<Screen.Screen> screenStack = new Stack<Screen.Screen>();
    } 
}
