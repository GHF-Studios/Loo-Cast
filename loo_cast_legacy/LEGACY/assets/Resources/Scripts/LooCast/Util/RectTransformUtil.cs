using System;
using UnityEngine;

namespace LooCast.Util
{
    public static class RectTransformUtil
    {
        public static Vector2 ScreenToRectPos(Vector2 screenPosition, RectTransform rectTransform, Canvas canvas)
        {
            if (canvas.renderMode != RenderMode.ScreenSpaceOverlay)
            {
                if (canvas.worldCamera == null)
                {
                    throw new NullReferenceException("Canvas has no camera assigned!");
                }
                RectTransformUtility.ScreenPointToLocalPointInRectangle(rectTransform, screenPosition, canvas.worldCamera, out Vector2 anchorPos);
                return anchorPos;
            }
            else
            {
                Vector2 anchorPos = screenPosition - new Vector2(rectTransform.position.x, rectTransform.position.y);
                anchorPos = new Vector2(anchorPos.x / rectTransform.lossyScale.x, anchorPos.y / rectTransform.lossyScale.y);
                return anchorPos;
            }
        }
    }
}
