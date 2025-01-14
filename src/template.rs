use crate::utils::camel_to_snake_case;

pub fn generate_usecase_content(module: &str, endpoint: &str) -> String {
    format!(
        r#"package {module}

import (
	"context"
	"go.uber.org/zap"
)

type {endpoint} interface {{
	Execute(ctx context.Context, request {endpoint}Request) (*{endpoint}Response, error)
}}

type {endpoint}Request struct {{
}}

func (req {endpoint}Request) Validate() error {{
	return nil
}}

type {endpoint}Response struct {{
}}

type {endpoint}Impl struct {{
	logger *zap.Logger
}}

func New{endpoint}(
	logger *zap.Logger,
) {endpoint} {{
	return &{endpoint}Impl{{
		logger: logger,
	}}
}}

func (uc *{endpoint}Impl) Execute(ctx context.Context, request {endpoint}Request) (*{endpoint}Response, error) {{
	if err := request.Validate(); err != nil {{
		return nil, err
	}}

	return &{endpoint}Response{{}}, nil
}}
"#
    )
}

pub fn generate_test_content(module: &str, endpoint: &str) -> String {
    format!(
        r#"package {module}

import (
    "context"
    "testing"
    "go.uber.org/zap"
    "github.com/stretchr/testify/assert"
)

func Test{endpoint}_ValidRequest(t *testing.T) {{
    ctx := context.Background()
    logger := zap.NewNop()
    uc := New{endpoint}(logger)
    
    resp, err := uc.Execute(ctx, {endpoint}Request{{}})

    assert.NoError(t, err)
}}
"#
    )
}

pub fn generate_errors_content(module: &str) -> String {
    format!(
        r#"package {module}

import "errors"

var (
    ErrSample = errors.New("sample error")
)
"#
    )
}

pub fn generate_codes_content(module: &str) -> String {
    format!(
        r#"package {module}

var (
    CodeSample = "U1"
)
"#
    )
}

pub fn generate_rest_content(module: &str, endpoint: &str, project_name: &str) -> String {
    let import_module = camel_to_snake_case(module);
    format!(
        r#"package {module}

import (
    "errors"
    "net/http"

    "github.com/gin-gonic/gin"
    "go.uber.org/zap"
    "{project_name}/src/core/entities"
    "{project_name}/src/core/usecases/{import_module}"
    "{project_name}/src/entrypoints/rest"
)

type {endpoint} struct {{
    useCase {module}.{endpoint}
    logger  *zap.Logger
}}

type {endpoint}Response struct {{
}}

func New{endpoint}(useCase {module}.{endpoint}, logger *zap.Logger) {endpoint} {{
    return {endpoint}{{useCase: useCase, logger: logger}}
}}

func (h {endpoint}) Handle() gin.HandlerFunc {{
    return func(c *gin.Context) {{
        req, err := h.bindRequest(c)

        if err != nil {{
            c.JSON(http.StatusBadRequest, rest.Errf("U0", "failed to bind request"))
            return
        }}

        resp, err := h.useCase.Execute(c, req)

        if err != nil {{
            status, code, message := rest.ErrResponse()
            switch {{
            case errors.Is(err, {module}.ErrSample):
                status, code, message = http.StatusBadRequest, CodeSample, "sample api error"

            default:
                h.logger.Error("{endpoint} failed", zap.Error(err), zap.Any("request", req))
            }}

            c.JSON(status, rest.Errf(code, message))
            return
        }}

        c.JSON(http.StatusOK, {endpoint}Response{{}})
    }}
}}

func (h {endpoint}) bindRequest(c *gin.Context) ({module}.{endpoint}Request, error) {{
    var req {module}.{endpoint}Request

    return req, nil
}}
"#
    )
}

pub fn generate_container_content(module: &str, endpoint: &str, project_name: &str) -> String {
    let import_module = camel_to_snake_case(module);
    format!(
        r#"package entrypoints

import (
    {import_module}h "{project_name}/src/entrypoints/rest/{import_module}"
)

type {module}Container struct {{
    {endpoint} {module}h.{endpoint}
}}

func New{module}Container({endpoint} {module}h.{endpoint}) {module}Container {{
    return {module}Container{{
        {endpoint}: {endpoint},
    }}
}}
"#
    )
}
