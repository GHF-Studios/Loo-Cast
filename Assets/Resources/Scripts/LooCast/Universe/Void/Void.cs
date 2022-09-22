using UnityEngine;
using System;

namespace LooCast.Universe.Void
{
    public class Void
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            // How many voids fit into the Universe (Per Axis)
            public int amount;
        }
        #endregion

        public Vector2 OffsetVoidPosition => voidPosition + normalizedVoidPositionOffset;
        public Vector2Int VoidPosition => voidPosition;

        [SerializeField] private Universe.GenerationSettings universeGenerationSettings;
        [SerializeField] private GenerationSettings generationSettings;
        [SerializeField] private Vector2Int voidPosition;
        [SerializeField] private Vector2 normalizedVoidPositionOffset;

        public Void(Universe.GenerationSettings universeGenerationSettings, Vector2Int voidPosition, Vector2 normalizedVoidPositionOffset)
        {
            this.universeGenerationSettings = universeGenerationSettings;
            generationSettings = universeGenerationSettings.voidGenerationSettings;
            this.voidPosition = voidPosition;
            this.normalizedVoidPositionOffset = normalizedVoidPositionOffset;
        }
    }
}