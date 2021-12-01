package main

type TxMgr interface {
	BeginTx(ctx context.Context, txFunc func(tx *sqlx.Tx) error) error
}

type SqliteManager struct {
	db *sqlx.DB
}

func (dbm *SqliteManager) BeginTx(ctx context.Context, txFunc func(tx *sqlx.Tx) error) (err error) {
	tx, err := dbm.db.BeginTxx(ctx, &sql.TxOptions{})
	if err != nil {
		return err
	}

	defer func() {
		if e := recover(); e != nil {
			_ = tx.Rollback()
		} else if err != nil {
			_ = tx.Rollback()
		} else {
			_ = tx.Commit()
		}
	}()

	err = txFunc(tx)
	return
}

// func (s *server) DeletePipeline(ctx context.Context, req *api.DeletePipelineRequest) (*api.DeletePipelineResponse, error) {
// 	pState, ok := s.pipelines[req.PipelineId]
// 	if !ok {
// 		return nil, status.Errorf(codes.NotFound, "pipeline %s not found", req.PipelineId)
// 	}
// 	capi.StopPipeline(pState)

// 	err := s.dbm.BeginTx(ctx, func(tx *sqlx.Tx) error {
// 		if err := s.dbm.TxDeletePipeline(ctx, tx, req.GetPipelineId()); err != nil {
// 			return err
// 		}
// 		if err := s.dbm.TxDeletePipelineFunctionAll(ctx, tx, req.GetPipelineId()); err != nil {
// 			return err
// 		}
// 		if err := s.dbm.TxDeletePipelineSourceAll(ctx, tx, req.GetPipelineId()); err != nil {
// 			return err
// 		}
// 		if err := s.dbm.TxDeletePipelineSinkAll(ctx, tx, req.GetPipelineId()); err != nil {
// 			return err
// 		}

// 		return nil
// 	})

// 	if err != nil {
// 		return nil, status.Errorf(codes.Internal, "failed to delete pipeline: %v", err)
// 	}

// 	delete(s.pipelines, req.PipelineId)

// 	return &api.DeletePipelineResponse{}, nil
// }
