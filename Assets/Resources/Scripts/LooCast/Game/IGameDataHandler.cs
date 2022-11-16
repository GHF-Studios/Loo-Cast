using System;

namespace LooCast.Game
{
    public interface IGameDataHandler
    {
        RuntimeData GetData();
        void SetData(RuntimeData data);
    }
}
