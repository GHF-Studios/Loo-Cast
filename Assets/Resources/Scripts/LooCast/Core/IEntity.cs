namespace LooCast.Core
{
    public interface IEntity
    {
        #region Properties
        IData Data { get; }
        ILogic Logic { get; }                                                                                                                                                                                                                                               
        #endregion
    }
}