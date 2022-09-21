using UnityEngine;
using System;

namespace LooCast.Universe.Void
{
    public class Void
    {
        public Vector2 OffsetVoidPosition => voidPosition + normalizedVoidPositionOffset;
        public Vector2Int VoidPosition => voidPosition;

        [SerializeField] private Vector2Int voidPosition;
        [SerializeField] private Vector2 normalizedVoidPositionOffset;

        public Void(Vector2Int voidPosition, Vector2 normalizedVoidPositionOffset)
        {
            this.voidPosition = voidPosition;
            this.normalizedVoidPositionOffset = normalizedVoidPositionOffset;
        }
    }
}