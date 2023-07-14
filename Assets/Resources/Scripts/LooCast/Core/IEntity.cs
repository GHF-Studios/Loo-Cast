namespace LooCast.Core
{
    public interface IEntity
    {
        #region Properties
        string EntityID { get; }
        IData Data { get; }
        ILogic Logic { get; }
        #endregion
    }
}
