using System;

namespace LooCast.Core
{
    public interface IEntity
    {
        #region Properties
        Guid EntityID { get; }
        IData Data { get; }
        ILogic Logic { get; }                                                                                                                                                                                                                                               
        #endregion
    }
}